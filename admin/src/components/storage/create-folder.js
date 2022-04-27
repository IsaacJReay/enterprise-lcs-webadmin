import React, { useState } from "react";
import { Modal, Button, message, Input, Form } from "antd";
import axios from "axios";
import { FiX } from "react-icons/fi";
import { useForm } from "antd/lib/form/Form";

const CreateFolder = ({
  visible,
  handleCancel,
  handleOk,
  uuid,
  selected,
  fetchData,
}) => {
  // -------state management ---------------

  const [, setLoading] = useState(false);
  const [form] = Form.useForm();

  // -------token ----------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
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
          setLoading(true);
          message.success("Successful!");
          fetchData();
          form.resetFields();
          handleOk();
          setLoading(false);
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        console.log(err);
        message.error("Operation Failed! ");
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
