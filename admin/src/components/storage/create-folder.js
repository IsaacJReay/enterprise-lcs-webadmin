import React from "react";
import { Modal, Button, message, Input, Form } from "antd";
import axios from "axios";
import { FiX } from "react-icons/fi";
import Cookies from "js-cookie";

const CreateFolder = ({
  visible,
  handleCancel,
  handleOk,
  uuid,
  selected,
  fetchData,
}) => {
  // -------state management ---------------

  const [form] = Form.useForm();

  // -------token ----------
  const baseUrl = process.env.REACT_APP_API_URL;
  // const getToken = localStorage.getItem("token");
  const getToken = Cookies.get("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  //   // ------- create folder ---------

  const handleApply = (data) => {
    const inputData = {
      directory_name: data.folder_name,
      parent_directory: selected,
      drive_partuuid: uuid,
    };
    axios
      .post(
        `${baseUrl}/settings/storage/device/directory/creation`,
        inputData,
        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
        }
      )

      .then((res) => {
        if ((res.statusCode = 200)) {
          message.success("Successful!");
          form.resetFields();
          fetchData();
          handleOk();
        } else {
          message.error("Operation Failed! ");
        }
      })

      .catch((err) => {
        console.log(err);
      });
  };

  return (
    <React.Fragment>
      <Modal
        title="Create new folder!"
        visible={visible}
        onCancel={handleCancel}
        onOk={handleOk}
        closeIcon={<FiX className="close-icon" />}
        footer={null}
      >
        <Form layout="inline" onFinish={handleApply} form={form}>
          <Form.Item
            label="Folder name"
            name="folder_name"
            rules={[{ required: true, message: "Folder name is required!" }]}
          >
            <Input
              placeholder="text here ..."
              size="large"
              className="input-info"
            />
          </Form.Item>
          <Form.Item>
            <Button type="primary" htmlType="submit" className="button-apply2">
              Create
            </Button>
          </Form.Item>
        </Form>
      </Modal>
    </React.Fragment>
  );
};

export default CreateFolder;
